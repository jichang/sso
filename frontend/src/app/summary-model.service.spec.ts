import { TestBed } from '@angular/core/testing';

import { SummaryModelService } from './summary-model.service';

describe('SummaryModelService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: SummaryModelService = TestBed.get(SummaryModelService);
    expect(service).toBeTruthy();
  });
});
