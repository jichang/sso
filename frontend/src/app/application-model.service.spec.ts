import { TestBed, inject } from '@angular/core/testing';

import { ApplicationModelService } from './application-model.service';

describe('ApplicationModelService', () => {
  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [ApplicationModelService]
    });
  });

  it('should be created', inject([ApplicationModelService], (service: ApplicationModelService) => {
    expect(service).toBeTruthy();
  }));
});
