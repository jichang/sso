import { TestBed } from '@angular/core/testing';

import { GroupModelService } from './group-model.service';

describe('GroupModelService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: GroupModelService = TestBed.get(GroupModelService);
    expect(service).toBeTruthy();
  });
});
