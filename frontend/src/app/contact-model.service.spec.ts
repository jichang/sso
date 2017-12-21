import { TestBed, inject } from '@angular/core/testing';

import { ContactModelService } from './contact-model.service';

describe('ContactModelService', () => {
  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [ContactModelService]
    });
  });

  it('should be created', inject([ContactModelService], (service: ContactModelService) => {
    expect(service).toBeTruthy();
  }));
});
